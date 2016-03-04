#lang racket

(require yaml)

(yaml-struct game-state (player rooms current-room))
(yaml-struct player (name health cpu))
(yaml-struct room (id description adjacent))

(define (add-room rooms room)
  (hash-set! rooms (room-id room) room))

(define (get-room rooms room)
  (hash-ref rooms room))

(define (enter-room state room-id)
  (begin
    (show "------------------------")
    (show (room-description (get-room (game-state-rooms state) room-id)))
    (struct-copy game-state state [current-room room-id])))

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

(define (display-player player)
  (begin
    (show (player-name player))
    (show "HP" (player-health player))))

(define (create-character-loop state)
  (begin
    (show "You're next up in line.")
    (show "Customs Officer: What is your name?")
    (let ([name (prompt "name> ")])
      (begin
        (show "Customs Officer: Very well, proceed on.")
        (game-loop
         (enter-room
          (struct-copy game-state state [player (player name 100 100)])
          "intro"))
        )
      )
    )
  )

(define (game-loop state)
  (begin
    (let ([command (prompt)])
      (match command
        ["help" (begin
                  (show "Help")
                  )]
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
