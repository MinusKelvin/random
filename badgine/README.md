# badgine

A chess engine designed to play poorly (i.e. like me).

* Random game tree searching makes it sometimes see tactics and sometimes not
* Eval primarily based on piece material value
* Has some idea that pushing pawns and restricting the opponent's king is good
  to make it not be braindead in endgames, but these also seem to make it very
  aggressive with its pawns and queen in the opening/midgame

The most unrealistic thing about this engine as a "bad player" is that it never
misses 1-ply tactics, such as mate-in-one and taking a hanging pieces.
