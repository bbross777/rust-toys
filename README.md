# rust-toys
Small programs I make while learning Rust.

### Goofspiel
This is a very basic implementation of the card game Goofspiel.

Two players each hold a suit of cards.
A third suit serves as the "prize" pile.

A card is drawn from the prize pile and shown to both players.
Each player "bids" in secret on the "prize" card with the cards in their hand.
When both bids are placed, the bids are revealed, and the player with the higher bid takes the prize.

The player with a greater sum of "prize" cards after 13 rounds (i.e. all cards have been played) wins.

The computer opponent in this implementation currently bids completely randomly.
