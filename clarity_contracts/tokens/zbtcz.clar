// File: tokens/zbtcz.clar

(define-fungible-token zbtcz-token u1000000)

(define-data-var locked-btcz uint u0)
(define-data-var finalized-states (list 100 {block-height: uint, merkle-root: (buff 32)}))
(define-data-var burned-records (list 100 {tx-id: uint, amount: uint}))

;; Optimized Minting zBTCZ
(define-public (mint-zbtcz (amount uint) (block-height uint) (merkle-root (buff 32)))
  (begin
    ;; Validate the Merkle root efficiently
    (asserts! (any?
                (fn (state)
                    (and (is-eq block-height (get block-height state))
                         (is-eq merkle-root (get merkle-root state))))
                (var-get finalized-states))
             (err u1002))
    ;; Update locked-btcz amount
    (var-set locked-btcz (+ (var-get locked-btcz) amount))
    ;; Mint fungible zBTCZ tokens
    (ft-mint? zbtcz-token amount (tx-sender))
    (ok "zBTCZ minted")))

;; Burning zBTCZ and Unlocking BTCZ
(define-public (burn-zbtcz (amount uint) (tx-id uint))
  (begin
    ;; Validate the amount to burn
    (asserts! (>= (ft-get-balance zbtcz-token (tx-sender)) amount) (err u1003))
    ;; Record the burned zBTCZ transaction
    (var-set burned-records (append (var-get burned-records) [{tx-id: tx-id, amount: amount}]))
    ;; Burn the zBTCZ tokens
    (ft-burn? zbtcz-token amount (tx-sender))
    ;; Decrease locked-btcz amount
    (var-set locked-btcz (- (var-get locked-btcz) amount))
    (ok "zBTCZ burned and BTCZ unlocked")))

;; Finalize State Update
(define-public (finalize-state (block-height uint) (merkle-root (buff 32)))
  (begin
    ;; Add the finalized state to the list
    (var-set finalized-states (append (var-get finalized-states) [{block-height: block-height, merkle-root: merkle-root}]))
    (ok "State finalized")))
