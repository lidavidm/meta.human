#lang racket

(require yaml)

(yaml-struct player (name health cpu))

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

(define (create-character-loop)
  (begin
    (show "You're next up in line.")
    (show "Customs Officer: What is your name?")
    (let ([name (prompt "name> ")])
      (begin
        (show "Customs Officer: Very well, proceed on.")
        (game-loop (player name 100 100))
        )
      )
    )
  )

(define (game-loop character)
  (let ([command (prompt)])
    (match command
      ["help" (begin
               (show "Help")
               )]
      ["status" (display-player character)])
    (game-loop character)
    ))

(define (mainloop)
  (call-with-continuation-prompt (lambda ()
                                   (parameterize ([prompt-parameter default-prompt]
                                                  [show-parameter default-show])
                                     (create-character-loop)))))

(mainloop)
