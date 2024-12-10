// File: clarity/governance.clar

(define-fungible-token gBTCZ u1000000)

(define-data-var total-staked uint u0)
(define-data-var reward-distribution-timestamp uint u0)
(define-data-var staker-info (map principal {stake: uint, rewards: uint}))
(define-data-var validator-info (map principal {locked-btcz: uint, rewards: uint, active: bool, last-proof-height: uint}))

(define-data-var total-validators uint u0)
(define-data-var reward-distribution-frequency uint u8)
(define-data-var unstaking-period uint u48)
(define-data-var proof-interval uint u16)

(define-data-var proposals (map uint {creator: principal, description: (buff 100), votes-for: uint, votes-against: uint, executed: bool, parameter-change: {param: (buff 20), value: uint}}))
(define-data-var next-proposal-id uint u1)

;; Optimized Staking Logic
(define-public (stake-btcz (amount uint))
  (begin
    (asserts! (> amount u0) (err u1001))
    ;; Efficiently update staker info
    (let ((current-stake (default-to u0 (get stake (map-get? staker-info tx-sender)))))
      (map-set staker-info tx-sender {stake: (+ current-stake amount), rewards: u0}))
    (var-set total-staked (+ (var-get total-staked) amount))
    (ok "Staking successful")))

;; Optimized Reward Distribution
(define-public (distribute-rewards)
  (begin
    (asserts! (>= block-height (+ (var-get reward-distribution-timestamp) (var-get reward-distribution-frequency))) (err u1103))
    (var-set reward-distribution-timestamp block-height)
    ;; Batch reward distribution
    (map
      (fn (staker principal)
        (let ((staker-data (unwrap! (map-get? staker-info staker) (err u1104))))
          (let ((stake (get stake staker-data)))
            (map-set staker-info staker {stake: stake, rewards: (+ (get rewards staker-data) (/ (* stake u100) (var-get total-staked)))}))))
      (map-keys (map staker-info)))
    (ok "Rewards distributed")))

;; Optimized Proposal Execution
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
            "reward-frequency" (var-set reward-distribution-frequency value)
            "unstaking-period" (var-set unstaking-period value)
            "proof-interval" (var-set proof-interval value)
            (err u2017))))
      (map-set proposals proposal-id (merge proposal {executed: true}))
      (ok "Proposal executed")))))

;; Read-Only Queries
(define-read-only (get-proposal (proposal-id uint))
  (ok (map-get? proposals proposal-id)))

(define-read-only (get-all-proposals)
  (ok (map proposals)))

(define-read-only (get-reward-frequency)
  (ok (var-get reward-distribution-frequency)))

(define-read-only (get-unstaking-period)
  (ok (var-get unstaking-period)))

(define-read-only (get-proof-interval)
  (ok (var-get proof-interval)))