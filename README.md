# Fishing Game!

by fishing game

## Team Members
* Advanced Topic Subteam 1: Physics/Fluid Dynamics
	* Cole Metrick
	* Ken Barrett
	* Jason Ye
	* Dheyab Alshehhi

* Advanced Topic Subteam 2: Bayesian Networks
	* Ben Gradeck
	* Jackie Colmenares
	* Apostoli Karpouzis
	* Corey Medve


## Game Description

We are making a top-down Fishing and exploration game. The player will explore the game world looking for new bodies of water to catch fish in. Along the way players will obtain new equiptment, sell their catches for money, and work towards catching the biggest fish they can. The gameplay will have two main Advanced topics to give some realism to the fishing. Fluid Dynamics will simulate casting the bait, using the lure, then dragging the fish in. It will simulate realistic fluic dynamics to give an accurate simulation of fishing. Once a fish is on the line the player will have to sucessfully reel a fish in without the fish snapping the line. The other advanced topic is sophisticated fish AI. There will be a determined number of fish in each water source. The fish in the game will all behave differently depending on species, ex: catfish staying exclusivly at the bottom, or trout only fishing against a streams current. New fish will spawn on a cycle to simulate fish reaching maturity or dying in order to simulate a target population. Enviornmental aspects and lure use will simulate catching fish, better lure use or casting into an ideal location will influence the fish to get hooked more often. 

### Concept Art:
![Artwork](art/art1.png)
![Artwork](art/art2.png)
![Artwork](art/art3.png)
![Artwork](art/art4.png)


## Advanced Topic Description

### Advanced Physics/Fluid Dynamics

Since our game is primarly played with a top down view, the physics will focus on the x and y axis with some z axis calculations for the baits (i.e. surface baits vs underwater baits). The translational and rotational motion of the fish will be calculated based on player actions, fish characteristics, and fishing rod characteristics. Fluid-dynamic drag will factor into the motion. Projectile motion will be incorpoated for lure casting based on the lure weight and rod strength. The tensile strength of the fishing rod and line will be taken into account for visuals and line snapping. For example, the rod will bend more when there is more tension on the line. Environmental factors like water currents and debris will affect tension on the line and increase drag.

### Bayesian Networks (AI)

Our goal with AI is to simulate the existence of how many fish are in the pond, and what types of fish they are, as well as their behavior. We will develop a behavior tree to model the variety of actions for each fish species. The fish behavior will be dependant on how close the lures are, and what kind of bait is in the water, as well as environmental conditions. These fish actions will be passed on to the fluid dynamics team. Depending on how the fish is moving, slowly or quickly, towards or away from the lure, will affect the water movement. The fish behavior will be calculated per frame, and the existance of those fish will be evaluated in a larger scope of the game world. 


## Midterm Goals

### General
* 6x6 grid
* 3 different lure types
* Player movement for overworld exploration
* Fluid dynamics for fishing mechanic
* Small collection of fish types with basic behavior/probability for spawning maybe 4 fish, 
* Shop system for buying lures and selling off fish
* Simple functional animation to convey gameplay Ideas 


### Physics
* Calculate fish drag force
  * Physical characteristics
    * Mass
    * Size
    * Shape
  * Motion
    * Velocity
    * Rotation
* Calculate force exerted by fish
   * Strength
   * Aggressiveness
* Implement player-fish-rod interaction
   * Reeling
   * Orientation of player and fish
    
### Bayesian Networks
* Create one fishing environment
  * Defined model for introduction and removal of fish into environment
  * Incorporate time of day and weather into environment's model/fish spawn probability
* Create two different fish types
  * Bayesian network for activity of the fish types
  * Take time of day, water depth, and weather into account for Bayesian networks (with fixed values for each)


## Final Goals

### General
* Different line types (braided, monofilament)
* Expanded collection of fish types and behaviors
* In water elements affecting spawning such as lily pads in ponds or rocks in water. 
* Weekly/ random shop deals/ fish
* Exploration objectives such as random chance to find rare lures in caught fishes or on the ground.
Polished animations that look cool


### Physics (20% total)
* Fishing rod (6%)
  * Line breaking
  * Rod bending
* Lure (6%)
  * Weight
  * Size
* Water current (4%)
* Debris (4%)

### Bayesian Networks (20%)
* 3 different fishing enviroments (5%)
  * Fishing enviroments attempt to maintain a population of fish with fish being added/removed each day 
* 5 different types of fish (5%)
* Implementation of day/night cycle (5%)
  * Look / lighting in the world changes based off of the time of day
  * The fish able to be caught and the activity of the enviroments differ on the time of day
* Implement weather cycle (5%)
  * Different regions of the map can have different weather at the same time
  * Weather can change throughout the day

## Stretch Goals (10%)

* Add 2 items that enhances the players ability to catch a fish (5%)
  * A pair of polarized glasses to improve the odds of finding fish
  * A new rod to change how the players interacts with the physics
* Add a legendary rarity fish (5%)
  * Much harder to catch and spawns less often than the other fish types
