//gonna have preferred time, weather, depth
//just notes

//i think we pregenerate an array of MAX_POND_SIZE

//we also pre generate an array of MAX_FISH ODDS size for each fish 
//we have separate arrays for all the individual species
//each array has all individual fish with their own traits

/*
if bayesian determines a 10% chance of catching bass, we select 10 
of the bass instances from the bass population array with highest hunger (or random if tie)
and populate the pond array at cast time.
same goes for all species.

if a fish is caught, increase age, decrease hunger dramatically

every in game day, increase both age and hunger
*/