(define-data-var locked-btcz uint 0)

(define-map token-balances (principal) uint)

(define-public (mint-zbtcz (amount uint))
    (begin
        ;; Ensure only the bridge can call this
        (asserts! (is-eq tx-sender (var-get bridge-operator)) (err u100))
        ;; Increase total locked BTCZ
        (var-set locked-btcz (+ (var-get locked-btcz) amount))
        ;; Increase the caller's zBTCZ balance
        (map-set token-balances tx-sender (+ (get-or-default (map-get? token-balances tx-sender) 0) amount))
        (ok true)))

(define-public (burn-zbtcz (amount uint))
    (begin
        ;; Ensure the user has enough zBTCZ to burn
        (let ((current-balance (get-or-default (map-get? token-balances tx-sender) 0)))
            (asserts! (>= current-balance amount) (err u101))
            ;; Reduce the user's zBTCZ balance
            (map-set token-balances tx-sender (- current-balance amount))
            ;; Decrease total locked BTCZ
            (var-set locked-btcz (- (var-get locked-btcz) amount))
            (ok true))))

(define-public (get-balance (user principal)) (ok (get-or-default (map-get? token-balances user) 0)))

(define-read-only (get-locked-btcz) (ok (var-get locked-btcz)))

(define-constant err-unauthorized u100)
(define-constant err-insufficient-balance u101)

(define-data-var bridge-operator principal tx-sender)
