// File: clarity/governance.clar

(define-fungible-token gBTCZ u1000000)

(define-data-var total-staked uint u0)
(define-data-var reward-distribution-timestamp uint u0)
(define-data-var validator-reward-rate uint u100) ;; Reward rate per block
(define-data-var staker-info (map principal {stake: uint, rewards: uint}))
(define-data-var validator-info (map principal {locked-btcz: uint, rewards: uint, active: bool, last-proof-height: uint}))

(define-data-var total-validators uint u0)
(define-data-var reward-distribution-frequency uint u8)
(define-data-var slashing-penalty uint u50) ;; Penalty for inactivity
(define-data-var proof-interval uint u16) ;; Blocks between required proofs

;; Validator Slashing Logic
(define-public (slash-validator (validator principal))
  (begin
    (let ((validator-data (map-get? validator-info validator)))
      (asserts! (is-some validator-data) (err u3001))
      (let ((data (unwrap! validator-data (err u3002))))
        (asserts! (not (get active data)) (err u3003))
        ;; Apply penalty
        (let ((penalty (min (get locked-btcz data) (var-get slashing-penalty))))
          (map-set validator-info validator {
            locked-btcz: (- (get locked-btcz data) penalty),
            rewards: (get rewards data),
            active: false,
            last-proof-height: (get last-proof-height data)
          })
          (var-set total-validators (- (var-get total-validators) u1))
          (ok "Validator slashed successfully."))))))

;; Validator Participation Check
(define-public (check-validator-activity (validator principal))
  (begin
    (let ((validator-data (map-get? validator-info validator)))
      (asserts! (is-some validator-data) (err u3004))
      (let ((data (unwrap! validator-data (err u3005))))
        (if (< (+ (get last-proof-height data) (var-get proof-interval)) block-height)
          (begin
            (map-set validator-info validator (merge data {active: false}))
            (err u3006))
          (ok "Validator is active."))))))

;; Governance Proposal Logic for Slashing Penalty
(define-public (propose-slashing-penalty-change (new-penalty uint))
  (begin
    (let ((proposal-id (var-get next-proposal-id)))
      (map-set proposals proposal-id {
        creator: tx-sender,
        description: "Change slashing penalty",
        votes-for: u0,
        votes-against: u0,
        executed: false,
        parameter-change: {param: "slashing-penalty", value: new-penalty}
      })
      (var-set next-proposal-id (+ proposal-id u1))
      (ok {proposal-id: proposal-id, description: "Change slashing penalty"}))))

(define-public (execute-proposal (proposal-id uint))
  (begin
    (let ((proposal (map-get? proposals proposal-id)))
      (asserts! (is-some proposal) (err u2010))
      (asserts! (not (get executed (unwrap! proposal (err u2011)))) (err u2012))
      (asserts! (>= (get votes-for (unwrap! proposal (err u2013))) (* u2 (get votes-against (unwrap! proposal (err u2014))))) (err u2015))
      (let ((change (get parameter-change (unwrap! proposal (err u2016)))))
        (let ((param (get param change))
              (value (get value change)))
          (match param
            "slashing-penalty" (var-set slashing-penalty value)
            (err u2017))))
      (map-set proposals proposal-id (merge proposal {executed: true}))
      (ok "Proposal executed")))))

;; Read-Only Queries
(define-read-only (get-validator-info (validator principal))
  (ok (map-get? validator-info validator)))

(define-read-only (get-validator-reward-rate)
  (ok (var-get validator-reward-rate)))

(define-read-only (get-slashing-penalty)
  (ok (var-get slashing-penalty)))
