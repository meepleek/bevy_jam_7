# Design doc

I actually dunno what a design doc is supposed to look like, this's just a bunch of thoughts about the entry

## Theme

**extremely incohesive feverdream**

### Other themes to add as well

- rats
- sentient environment - maybe???
- this is getting out of hand - just as a joke during an intro cutscene

## Story

- kid is wakes up with a fever describing their insane dream

### a) Pharmacy

- parents decide to send one of their kids to the pharmacy for a drug, but they forget their wallet home and need to play and beat a card game designed by the pharmacist => the game itself
    - bonus points to reuse the same art (comic-strip?) to retell different unhinged inside jokes
- use the waking up from a (fever)dream as the in-univers explanation for failure/death

### b) Battling the disease/fever

- thematically the enemies are the disease

### Nonsense to iclude from the stream/themes/jam chat

- ratzilla (& other Jan's rats)
- nutella
- penne (filled with nutella)
- colonoscopy
- offensive towers

### Other dumb jokes

- game is gonna be played on a Ratbook Pro
- the splash of the game should show Jan's rat bevy logo (Made with Mischief ?)
- loading screen lists dumb stuff like
    - filling penne with nutella
    - looking for colonoscopy pics
    - smearing nutella on a crying child
    - checking the dishwasher for rats
    
## Vibe to go for

- uneasy, confusing, surprising
- use different incohesive objects in the world/level - mainly for enemies
    - try to go for typical dream stuff like teeth

## Mechanics

- temp meter (UI styled as a thermometer) which can't go neither too high or too low
- heat - players set temp increase for each round
    - 1 => 2
    - 2 => 4
    - 3 => 7
    - 4 => 11
    - also increases enemy spawns?
- after each round
    - earn income for heat
    - buy cards
    - allow selling one card?
        - simplify UI by just giving a hand of cards to sell from
        - junk cards the player has to actually pay for instead 
    - set heat for next level
- allow players to carry over 1 card & 1 ability from a failed run
- push yr luck (some abilities tempting players to stay at low/high temp)

### Possible (focused) strategies

- regular attack-based
- push/pull based (hole-y)
- explosion-based
- income-based
- low-temp? (would need some cards/synergies)
    - free movement cards at low temp?
- high-temp? (would need some cards/synergies)
    - free movement cards at high temp?

### Cards

#### Card uses

- main effect
    - cost - usually temp raise
- discard (not all cards)
- trash (not all cards)

#### Card types

- temp 2-3
    - lower
    - raise
- move 1-2
    - diag
    - orthogonal
    - all directions
- attack 1-2
    - line
    - lob (over walls)
    - diag
    - orthogonal
- pull/push 2-3
    - diag
    - orthogonal
    - affects coins as well (can be pulled in)
- income (nutella?)
    - +1 coin
    - coin to attack
    - coin to move
- explosion
    - lay mine
        - explodes all around
        - makes a hole on the tile
        - destroys adjacent walls
    - detonate any mine + trash 1 card
    - next kill this turn will explode
- holes
    - make a hole & trash 1 card

#### Inspo matrix

| xxxxxxxxx | temp | move | attack | lob | pull/push | income | explosion | holes | draw |
| --------- | ---- | ---- | ------ | --- | --------- | ------ | --------- | ----- | ---- |
| temp      |      |      |        |     |           |        |           |       |      |
| move      |      |      |        |     |           |        |           |       |      |
| attack    |      |      |        |     |           |        |           |       |      |
| lob       |      |      |        |     |           |        |           |       |      |
| pull/push |      |      |        |     |           |        |           |       |      |
| income    |      |      |        |     |           |        |           |       |      |
| explosion |      |      |        |     |           |        |           |       |      |
| holes     |      |      |        |     |           |        |           |       |      |
| draw      |      |      |        |     |           |        |           |       |      |

#### Start deck (default character at least)

- movement, close-range attacks, temp +1 x 2, temp -2
- might differ for other characters

### Abilities/characters

- +1 range for attacks
- +1 range for movement
- spend coin to change temp +/- 2
- +1 to hand size
- explosion immunity
- -1 temp cost for movement cards at min temp

### Enemies

- enemies attack only when the player is in range

### Types

- short range (all neigbour tiles)
- range 3 diag/ortho
- lobbing
- explosive enemies that rush the player
- enemies that change temp
- enemies that add temp/junk cards
- enemies that protect others/add single hit shields

#### Inspo matrix

| xxxxxxxxx | direct | lob | explosive | temp | junk | shield |
| --------- | ------ | --- | --------- | ---- | ---- |        |
| direct    |        |     |           |      |      |        |
| lob       |        |     |           |      |      |        |
| explosive |        |     |           |      |      |        |
| temp      |        |     |           |      |      |        |
| junk      |        |     |           |      |      |        |
| shield    |        |     |           |      |      |        |

### To consider

- turn timer to avoid AP?
- multi-use cards
    - each effect is single-use and gets removed from the card
    - each effect has to be used to refresh all effects of the card

### Visuals

- shape-shifting/morphing (might need to go vector/svg for that)
- agressively neon-colored
- bloom
- card elements should shift
- text can be shaky/moving as well
- aim for accessibility options for the moving stuff
