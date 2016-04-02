#lang racket

(require yaml)

(yaml-struct game-state (player rooms current-room))
(yaml-struct player (name health cpu))
(yaml-struct room (id description adjacent))

(define (add-room rooms room)
  (hash-set! rooms (room-id room) room))

(define (get-room rooms room)
  (hash-ref rooms room))

(define (default-prompt [text "meta.human> "])
  (let ([result (begin
                  (display text)
                  (read-line))])
    (if (eof-object? result)
        (abort-current-continuation
         (default-continuation-prompt-tag)
         (lambda () 0))
        result)))
(define (default-show . text)
  (begin
    (for ([item text])
      (display item)
      (display " "))
    (newline)))
(define prompt-parameter (make-parameter default-prompt))
(define show-parameter (make-parameter default-show))

(define (prompt [text "meta.human> "])
  ((prompt-parameter) text))
(define (show . args)
  (apply (show-parameter) args))
(define (say name . args)
  (apply show (cons (string-append name ":") args)))
(define (title title)
  (let ([underline (build-string (string-length title) (lambda (_) #\*))])
    (begin
      (show underline)
      (show title)
      (show underline)
      )))

(define (display-player player)
  (begin
    (show (player-name player))
    (show "HP" (player-health player))))

(define (describe-room room)
  (begin
      (show (room-description room))
      (cond
        [(not (empty? (room-adjacent room)))
         (for ([connection (room-adjacent room)])
           (show (first connection)))])))

(define (enter-room state room-id)
  (let ([room (get-room (game-state-rooms state) room-id)])
    (begin
      (describe-room room)
      (struct-copy game-state state [current-room room-id]))))

(define (create-character-loop state)
  (begin
    (show "You're next up in line.")
    (say "Customs Officer" "What is your name?")
    (let ([name (prompt "name> ")])
      (begin
        (say "Customs Officer" "Very well, proceed on.")
        (game-loop
         (enter-room
          (struct-copy game-state state [player (player name 100 100)])
          "intro"))
        )
      )
    )
  )

(define (get-command)
  (let* ([command (prompt)]
         [parts (string-split command)])
    (if (empty? parts)
        (get-command)
        (cons (string-downcase (first parts)) (rest parts)))))

(define (help)
  (begin
    (title "Help")))

(define (describe state [args '()])
  (if (empty? args)
      (describe-room (get-room (game-state-rooms state) (game-state-current-room state)))
      (show "I don't understand.")))

(define (game-loop state)
  (begin
    (let ([command (get-command)])
      (match (first command)
        ["help" (help)]
        ["describe" (describe state)]
        ["status" (display-player (game-state-player state))]
        [_ (show "Invalid command.")])
      (game-loop state)
      )))

(define (mainloop)
  (define rooms (make-hash))
  (add-room rooms (room "intro" "Description" null))
  (define initial-state (game-state (player "" 100 100) rooms (get-room rooms "intro")))
  (call-with-continuation-prompt (lambda ()
                                   (parameterize ([prompt-parameter default-prompt]
                                                  [show-parameter default-show])
                                     (create-character-loop initial-state)))))

(mainloop)
