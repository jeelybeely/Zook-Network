// File: contracts/bridge.clar

(define-data-var locked-tokens uint u0)
(define-data-var burned-tokens uint u0)
(define-data-var total-supply uint u0)

(define-public (lock-tokens (amount uint) (sender principal))
  (begin
    (var-set locked-tokens (+ (var-get locked-tokens) amount))
    (var-set total-supply (+ (var-get total-supply) amount))
    (ok { message: "Tokens locked", amount: amount, sender: sender })))

(define-public (burn-tokens (amount uint) (sender principal))
  (begin
    (if (<= amount (var-get total-supply))
        (begin
          (var-set burned-tokens (+ (var-get burned-tokens) amount))
          (var-set total-supply (- (var-get total-supply) amount))
          (ok { message: "Tokens burned", amount: amount, sender: sender }))
        (err "Insufficient total supply"))))

(define-read-only (get-locked-tokens)
  (ok (var-get locked-tokens)))

(define-read-only (get-burned-tokens)
  (ok (var-get burned-tokens)))

(define-read-only (get-total-supply)
  (ok (var-get total-supply)))
