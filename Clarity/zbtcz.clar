// File: clarity/zbtcz.clar

(define-non-fungible-token zbtcz-id uint)

(define-data-var locked-btcz uint u0)
(define-data-var finalized-states (list 100 {block-height: uint, merkle-root: (buff 32)}))
(define-data-var burned-records (list 100 {tx-id: uint, amount: uint}))

;; Optimized Minting zBTCZ
(define-public (mint-zbtcz (amount uint) (block-height uint) (merkle-root (buff 32)))
  (begin
    ;; Validate the Merkle root efficiently
    (asserts! (any?
                (fn (state)
                  (and (is-eq (get block-height state) block-height)
                       (is-eq (get merkle-root state) merkle-root)))
                (var-get finalized-states))
              (err u1001))
    ;; Increment locked BTCZ in one operation
    (var-set locked-btcz (+ (var-get locked-btcz) amount))
    ;; Batch mint zBTCZ NFTs
    (ok (nft-mint-batch zbtcz-id tx-sender amount))))

;; Optimized Burning zBTCZ
(define-public (burn-zbtcz (ids (list 100 uint)))
  (begin
    ;; Verify ownership in a batch
    (asserts! (every? (fn (id) (is-eq (nft-get-owner? zbtcz-id id) (some tx-sender))) ids)
              (err u1002))
    ;; Batch burn zBTCZ NFTs
    (nft-burn-batch zbtcz-id ids)
    ;; Decrement locked BTCZ
    (var-set locked-btcz (- (var-get locked-btcz) (len ids)))
    ;; Log burn record
    (let ((tx-id (len (var-get burned-records))))
      (var-set burned-records (append (var-get burned-records) [{tx-id: tx-id, amount: (len ids)}])))
    (ok "Burn successful")))

;; Read-Only Queries
(define-read-only (get-locked-btcz)
  (ok (var-get locked-btcz)))

(define-read-only (get-finalized-states)
  (ok (var-get finalized-states)))

(define-read-only (get-burned-records)
  (ok (var-get burned-records)))
