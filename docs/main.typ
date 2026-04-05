= Campfire Creators Jam 2026

by Casey Walker and Grace Xin

== The concept

The main objective of the game: _keep your campfire alive_.

This is a 2.5D game where the player is plays a character that needs to keep the
  campfire from dying.
The campfire will burn through its fuel over time, so the player must move
  around the world and collect fuel to bring back to the campfire.

== Minimum goals

As a minimum for submission, we are planning to implement the following:

- a campfire with a time-based deteriorating meter, which can be fed fuel
  - the meter is a static HUD element (e.g. a bar)
  - the campfire does not visually change based on the meter level---it will
      always be the provided assets required by the game jam criteria
- collectable fuels that can be stored and fed to the campfire
  - sticks/twigs laying on the ground
  - wood logs that can be picked up from set locations
- a character that can move around the world and interact with fuels and the
    campfire
- environments for the player to travel between
  - the campsite (where teh fire is located)
  - a forest, where sticks and logs can be gathered

=== General tasks that can be parallelized

In general, we will can divide the work into the following parallelizable
  categories:

- campfire meter HUD
- player interaction, such as moving and picking up items
- building and setting up the campfire/environments
- inventory mechanism
