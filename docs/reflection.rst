.. _reflection:

Reflection
===========

Log Entry: June 8, 2025
------------------------

The inspiration behind this project stems from *Max Imaginations' Mini FPV Drone Tutorial*. 
Watching that video sparked the idea to create a similar design with my own custom modifications to better understand the process of engineering design.
Having recently graduated with a bachelor's degree in electrical engineering, I wanted to build more experiences in the field of electrical engineering. 
I found that by embarking on this project, I could apply my knowledge I gained in my studies to a practical application and also explore new concepts and ideas along the way.

It was not easy to commit to the project in the beginning because deep down I knew that I would be spending a lot of time and resources on this project even more so that this project
requires a lot of electrical and mechanical components that could be quite expensive. Furthermore, I am also working full-time as a software engineer and finding 
free time to work on this project was a challenge. Alot of the time spent on this project was during the weekends and late nights after work which required a lot of sacrifice. 

However, I was determined to see this project through to better myself as an electrical engineer and to create something new and challenging despite my lack of 
personal experiences in building drones or electrical engineering projects in general. 

The start of the project was slow as I had to first list all the electrical and mechanical components I needed to build the drone and its controller. I wanted to 
gather as much detail as possible such as the cost, size, and weight of each component to assess its viability for the project. Most of the components I had used 
remains the same as Max Imaginations' Drone, but I wanted my own custom design of the frame.

After listing the components for the drone and the controller, I had started to replicate the wooden frame design by Max Imagination, but after recreating the frame,
I found that it was too small and simple looking for my liking. I wanted a more streamlined and modern design so I explored other designs online that uses carbon fiber. 
I eventually skecthed out a design that I liked and draw a blueprint of the components for my frame as a cutout template. I then moved on to purchasing a 0.5mm thick carbon fiber sheet.
I printed the cutout template and made the cutouts on the carbon fiber sheet for each component of the frame using a dremel tool. The frame was eventually built
using carbon fiber and M2 screws and nuts to hold the components in place.

The next step was building the electrical components of the drone. I had to first study the circuit provided by Max Imaginations' Drone and I also redraw the circuit in EasyEDA for my own references.
After drawing the circuit schematics for the drone and the controller, I then purchased the components from Amazon, Digikey, and Mouser. This stage was also a bit slow as I had to 
wait for the components to arrive before I could continue with the project. There were a lot of delays and some parcels lots and I had to reorder some of the components
from different sources. 

Once all the electrical components arrived, I started building the circuit for the drone by soldering the components together. I had little experiences with soldering
electrical components so I had some challenges in this area. Primarily, the soldering iron oxidized quickly which is probably due to poor techniques I am employing or poor
solder material that I am using. I had to switch solder tips quite frequently, but this wasn't very efficient as I ran out of solder tips and I was forced
to use an ineffective soldering iron which lead to poor soldering quality. This stage started to take a bad turn for me as the poor soldering quality 
affected the overall quality of the design as the circuit was prone to short-circuiting and the components not working as expected.

I reached a stage where I had the circuit for the drone built and assembled and programmed the Arduino Nano, but there were lots of failures with MultiSim. The drone was able to 
be programmed, but it did not interface well with MultiSim as the MPU6050 Gyro movements was not translated well by the software which looked like a lot of noise happening. Furthermore,
the MPU only seemed to move digitally when grounding pin D12 on the Arduino Nano which was not expected. At this stage I was not satisfied with the current work on my drone as I am starting to
see failures and weaknesses in my design. At this point I had to step back and reflect on my design and make revisions to resolve the issues I was facing. I've documented
the issues with this design which I have indicated as "prototype one" under the `prototypes section`.

In this stage, I have planned to recreate the design with the revisions and name it as "prototype 1.1" for a better outcome as the previous design.

Log Entry: July 22, 2025
-------------------------

This log will provide details to the outcome of prototype 1.1 and the revisions made to the design. This prototype had a much more compact design 
with shorter connections and lighter wires. Furthermore, the controller had a proper enclosure recycled from a toy controller enclosure with modifications
to fit the new components. However, this prototype also failed the final testing. The initial work for this prototype started by assembling the electrical components by resoldering the drone circuit using 28 AWG silicon copper wires. 
After the drone circuit is complete, the next task was to modify an old toy controller enclosure to fit the new controller components. This task involved drilling and cutting
certain parts of the enclosure using a dremel tool. Once the enclosure was fitted, the new components were installed and soldered. 

In this stage, the drone circuit and the controller circuit are both assembled and programmed for testing. The first test on the drone had some success
in which the IMU was able to ready the gyro movements. However, the flight controls were unresponsive to the joystick movements of the controller. I found later
on that the radio modules from the controller were faulty. This test involved setting up a simple communication between two of the spare radio modules
on a breadboard. After reordering a new set of radio modules, I was able to get clear communication between these new modules. Moving forward, the drone
was also able to capture the signals sent from the controller as shown in MultiSim. Lastly, I was not fully confident that the drone enclosure was properly insulated,
so I tested the drone responding without it's enclosure. This testing was successful as the motors properly relayed the controller's signals with the expected
motor speed based on the throttle joystick movements. However, the drone failed once I tested with the drone enclosure attached. I suspect, this had shorted the drone
because of its carbon fiber frame. I had regretted not testing the insulation of the frame even with Kapton tape. Again at this point, I needed to step back again
and assess the failures of this prototype and make more revisions to resolve these issues. More details of this assessment under the `prototypes section`.

In this stage I have ordered replacements for the damaged components such as the Arduino Nano, Arduino Pro Mini, NRF24L01 radio modules, and the controller battery.
I have planned to recreate the design with the revisions and name it as "prototype 1.2" for a better outcome as the previous design.