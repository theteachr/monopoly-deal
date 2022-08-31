# Design

src: Player who played the card
tgt: Player against whom the card is played (in some cases, all the opponents
can be targeted)
sys: Game state

## Action Cards

`*` bullets indicate the checks that need to be done to proceed further.
If any of the check fails, the method should return the appropriate error. These checks
will not include the trivial ones that are related user input sanitization.

All action cards can be
- played in the center to activate the action.
- played into the bank as money (loses action: can only be played as money for
the rest of the game)

- Deal Breaker
	* There is at least one player with a complete set
	- src: Choose player
		* Chosen player must have a complete set
	- src: Choose a complete set from the chosen player
		* What if `src` already owns a color of the chosen complete set?
	- sys: Add the chosen set to `src`'s assets
- Debt Collector
	- src: Choose player
	- tgt: Choose cards from the assets to sum up to 5 M
	- sys: Add cards paid to `src`'s assets
- Double The Rent
	* A rent card must be played previously
	- sys:
		- Get the previously played rent card's `amount` and targets
		- Ask the targets `amount` one more time
	- tgt: Choose cards from the assets to sum up to `amount` M
	- sys: Add cards paid to `src`'s assets
- Forced Deal
	- src: Choose card
	- src: Choose player
	- src: Choose property from player's assets
		* `src` should not have a complete set of the property color
	- sys: Move chosen property into `src`'s assets
- Hotel
	- src: Choose property color
		* Chosen property should be a full set
		* Chosen property should not be railroads or utilities
	- sys: Add 4 M to the rent of the property set
- Hotel
	- src: Choose property color
		* Chosen property should be a full set
		* Chosen property should not be railroads or utilities
	- sys: Add 3 M to the rent of the property set
- Birthday
	- tgt: Choose cards from the assets to sum up to 2 M
	- sys: Add cards paid to `src`'s assets
- Just Say No
	* Can only be played when asked for rent
	- sys: Return empty collection of cards
- Pass Go
	- sys: Add two cards to `src`'s hand
- Sly Deal
	- src: Choose player
	- src: Choose any card that's not a complete from the player's assets

## Property Cards

Played into the player's assets.

- src: Choose card from hand
	* Chosen card must be of a color whose set is not complete
- sys: Add the chosen card to `src`'s assets

## Rent Cards

Played in the center to ask for rent on owned properties.

- src: Choose card from hand
	* Player owns at least one color prop listed on the card
	- src: Choose player if it's a wild rent card
- sys: Calculate `rent` for the color
- tgt: Choose cards from the assets to sum up to `rent` M
- sys: Add the chosen card to `src`'s assets

## Money Cards

- src: Choose card from hand
- sys: Add chosen card to `src`'s assets
