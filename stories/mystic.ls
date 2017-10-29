tavern-enter
 check player visited_tavern
  if true
   say Welcome to the tavern
  else
   say The tavern door swings open
 check player isDrunk
  if true
   say Back so soon? (bar tender)
  next tavern

tavern
 set play visited_tavern true
 select
  bar Have a drink
  tavern-look Look around
  tavern-exit Leave
 next restart

tavern-look
 say You see a bartender, taking her time to make drinks. She seems to have a sour face on.
 check player isDrunk
  if true
   say You see a cloaked figure sitting in the corner.
   confirm Talk to cloaked figure?
    if true
     next tavern-cloaked-figure
  next tavern

tavern-cloaked-figure
 say Keep it to yourself, fool!
 next tavern

tavern-exit
 say See ya, stranger (bar tender)
 next town

town
 say You see a small tavern
 select
  sleep Sleep?
  tavern-enter Enter tavern?
 next restart

sleep
 exit

root
 say Welcome to the story about the Mystic
 say You see a tavern with lights on and decide to enter
 now tavern-enter

bar
 say You take a gulp of a stiff drink
 say You immediately feel the effects
 set player isDrunk true
 next tavern
