// File: contracts/validator_rewards.clar

(define-data-var total-rewards uint u0)
(define-data-var validator-rewards (map principal uint) {})

(define-public (record-performance (validator principal) (activity-score uint))
  (begin
    (if (> activity-score u100)
        (err "Activity score cannot exceed 100"))
    (let ((reward (/ (* activity-score u1000) u100)))
      (map-set validator-rewards validator (+ (default-to u0 (map-get? validator-rewards validator)) reward))
      (var-set total-rewards (+ (var-get total-rewards) reward))
      (ok { validator: validator, reward: reward }))))

(define-public (distribute-rewards (validator principal))
  (begin
    (match (map-get? validator-rewards validator)
      none (err "No rewards for this validator")
      some reward
        (begin
          (map-delete validator-rewards validator)
          (var-set total-rewards (- (var-get total-rewards) reward))
          (ok { validator: validator, distributed-reward: reward }))))

(define-read-only (get-total-rewards)
  (ok (var-get total-rewards)))

(define-read-only (get-validator-reward (validator principal))
  (ok (default-to u0 (map-get? validator-rewards validator))))
