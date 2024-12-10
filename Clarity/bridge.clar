// File: clarity/bridge.clar

(define-data-var locked-transactions (list 100 {tx-id: uint, amount: uint}) [])

(define-public (lock-btcz (tx-id uint) (amount uint))
  (begin
    ;; Ensure the transaction is unique
    (asserts! (not (any? (fn (tx) (is-eq (get tx-id tx) tx-id)) (var-get locked-transactions)))
             (err u1001))
    ;; Add the transaction to the locked-transactions list
    (var-set locked-transactions (append (var-get locked-transactions) [{tx-id: tx-id, amount: amount}]))
    (ok "BTCZ locked")))

(define-public (unlock-btcz (tx-id uint))
  (begin
    ;; Find the transaction
    (let ((transactions (var-get locked-transactions)))
      (let ((filtered (filter (fn (tx) (not (is-eq (get tx-id tx) tx-id))) transactions)))
        ;; Update the locked-transactions list
        (asserts! (< (len filtered) (len transactions)) (err u1002))
        (var-set locked-transactions filtered)
        (ok "BTCZ unlocked")))))

(define-read-only (get-locked-transactions)
  (ok (var-get locked-transactions)))
