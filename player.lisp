update:

event_damage = 0

health = (clamp_integrate (dot event_damage damage_effectiveness) (.. 0 max_max_health))


health = clamp[health+health_change,  range[0, max_health]]

consciousness = curve[brain_health_consciousness_curve, clamp brain_health]

blood > 0


apoxic


oxygen_in = breathing * lung_effectiveness * environment_air_content

( blood_oxygen = (clamp  (integrate (- oxygen_in oxygen_used) ))


rest = (clamp integrate)


consciousness = limit consciousness_sleep



if sleep
        integrate rest  
    clamp rest
else

end


alive = consciousness > 0