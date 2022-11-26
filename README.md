# Lucky Nine


Lucky 9 is a scored game like Baccarat and played like Blackjack.
The object of the game is to beat the cpu with a higher valued cards and as close to 9 as possible.

## The values of each card is listed below

1. Tens and face cards 10 points but considered as the lowest card value 
2. aces are worth 1 point
3. and 2-9 are scored as a regular value.

Hands are valued as the last digit of their sum.

## How to run the game:

In the termninal enter the following command:

```bash
   cargo run src/main.rs
```

## Game Mechanics

At the start of the game the player will have a starting balance of $2000.
The player is able to bet using his/her balance.
The the betting phase is done a player will receive 2 set of card and will have the total value showed. 
The player can choose to draw another card or show the card and match the opponents card. (Note: Drawing an extra card can only use once)
Once all the card are shown, The game will determine who is the winner.
If the player wins, he will rewarded by the amount that he bet. If player lost, player lost the bet as well.
The game will continue to run unless there is no more balance available for the player or the player decided to quit.
