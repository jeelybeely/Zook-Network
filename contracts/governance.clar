// File: contracts/governance.clar

(define-data-var minimum-stake uint u1000)
(define-data-var activity-threshold uint u80)
(define-data-var vote-duration uint u1440) ;; Duration in blocks
(define-data-var current-proposal (optional (tuple (param-name (string-ascii 32)) (value uint))) none)
(define-data-var votes-for uint u0)
(define-data-var votes-against uint u0)

(define-public (propose (param-name (string-ascii 32)) (value uint))
  (begin
    (if (is-none (var-get current-proposal))
        (begin
          (var-set current-proposal (some { param-name: param-name, value: value }))
          (var-set votes-for u0)
          (var-set votes-against u0)
          (ok "Proposal submitted"))
        (err "A proposal is already active"))))

(define-public (vote (vote-for bool))
  (begin
    (match (var-get current-proposal)
      none (err "No active proposal")
      some proposal
        (if vote-for
            (begin (var-set votes-for (+ (var-get votes-for) u1)) (ok "Voted for"))
            (begin (var-set votes-against (+ (var-get votes-against) u1)) (ok "Voted against"))))))

(define-public (finalize-proposal)
  (begin
    (match (var-get current-proposal)
      none (err "No active proposal")
      some proposal
        (if (>= (var-get votes-for) (var-get votes-against))
            (begin
              (if (is-eq proposal.param-name "minimum-stake")
                  (var-set minimum-stake proposal.value)
                  (ok "Parameter not recognized"))
              (var-set current-proposal none)
              (ok "Proposal approved"))
            (begin
              (var-set current-proposal none)
              (ok "Proposal rejected"))))))
