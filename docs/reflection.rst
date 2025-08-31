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

Log Entry: August 18, 2025
---------------------------

This log will provide details to the outcome of prototype 1.2 and the revisions needed to be made for the design. After implementing this prototype, the behaviour of the drone
was quite erratic. The drone kept resetting when throttle is decreased near zero. It seems to keep resetting and attempts to recalibrate the IMU. The motor responses were rare and seldom spins. When the motor does respond, it 
responds in bursts and then the drone resets. Looking at the MultiSim outputs, the AUX1 and AUX2 were correctly translated including the joystick movements for roll, pitch, yaw, and throttle levels. So I don't think the issue
has to do with the controller, rather with the drone itself. 

I suspect possible issues with the drone circuit listed below.

1. The battery discharge rate is too low (25C) and that a proper drone battery with a higher discharge rate (30C or higher) is needed.
2. The power for the radio is not consistent and requires a 10uF filtering capacitor at the NRF24 power inputs. 
3. The power for the Arduino Pro Mini is not consistent and requires a 100uF filtering capacitor. 
4. The power lines has a large AWG (small thickness) where the current cannot be supplied properly.
5. Motor PWM signals could be too weak to drive the motors. 
6. Potential EMF noise or leaks is affecting the IMU readings? 

Looking for a battery replacement greater than 25C was quite a challenge as most of the batteries I found were 25C or lower. It was recommended to use "Turnigy Nano-Tech" batteries,
but these were always not available and kept being out of stock due to their high demand for the high performance. Although I did manage to find a 30C LiPO battery, not from Turnigy, but from AMZZN, it still didn't
resolve the main issue. The drone behaved a bit better, but most of the resetting issue persisted. 

I moved forward by adding filtering capacitors at the radio module and Arduino Pro Mini power inputs, but this did not resolve the issue. I also ensured the solder for the power connections and the 
radio connections were solid and well connected, but this also did not resolve the issue. I even created a new motor driver with new MOSFETs and diodes, but this still did not resolve the issue. 

I then suspected, it has to do with the software and that the PWM signals were too weak to drive the motors. Unfortunately, I do not have an oscilloscope to measure the PWM signals properly, but I found the factor in the MultiWii software
that tunes the PWM outputs. Following the tutorial by Max Imaginations, the variable should be `float adjustmentFactor = 0.255` in line 1069 of output.cpp. I set this value to 0.255 from the default of 1, but this did not resolve the issue.
As a future work, it seems that it will be best to purchase an oscilloscope to confirm this suspicion. 

Moving forward, I retraced my steps and assessed the instructions for properly arming the drone using MultiSim. First the AUX1 and AUX2 should be set high for arming the drone and the beeper respectively. Once set, the drone accelerometer should
be calibrated and the read and write to save the changes. To arm and calibrate the drone, the AUX1 should be set low and the throttle and pitch controls are set to zero until the calibration noises are in effect. Once in effect, wait atleast 3 seconds
to apply the calibrations. With these steps, the drone still didn't manage to respond. 

My next suspicion is the hardware itself. I purchased the Arduino Pro Mini from "Hutomwua" which did not work, I intend to purchase the Arduino Pro Mini from "Robojax" which worked in the previous prototype (v1.1) where the motors actually responded. 
I have purchased the same Arduino Pro Mini and will try again in the next version. Furthermore, I will confirm the specifications for the MOSFETs, diodes, and resistors are properly rated for the circuit and the power requirements. More details of this assessment under the `prototypes section`.
I have planned to recreate the design with the revisions and name it as “prototype 1.3” for a better outcome as the previous design.

Log Entry: August 31, 2025
---------------------------

This log will describe the details to the outcome of prototype 1.3 and the revisions made to the design. Based on the last suspicion that the hardware purchase for the Arduino Pro Mini
was the source of the issue, this suspicion turned out to be true. Looking at the Arduino Pro Mini purchased from "Hutomwua", the board frequency was only 10MHz. This did not meet specifications 
for a 5V 16MHz board. I repurchased the Arduino Pro Mini from "Robojax" which was 5V 16MHz and this resolved the issue. The drone was able to respond to the controller's joystick movements.

However, the design is still not perfect as further issues continued. Although the motors responds to the joystick movements, the drone behaves erratically unable to make a proper liftoff.

The possible issues are listed below.

1. The drone is not calibrated properly.
    - The drone needs to sit on a flat surface for a proper calibration.
    - The accelerometer and the gyroscope needs proper calibration.
    - Adjust settings in MultiWii configuration with max smoothness.
2. Motor direction is wrong.
    - This can be verified by feeling if the air is being pushed upwards.
    - Record slow motion video to see the direction of the motors.
3. The motor RPMs are not the same and unsynchronized.
4. The forward direction of the MPU6050 is in the opposite direction. 
    - Rewire orientation of the motors to have face the MPU6050 in its forward direction.
5. The drone is still too heavy and certain weights of the components are not balanced causing the center of gravity to be offset.
    - Remove heavy motor mounts and just rely on superglue to attach the motors.



Key Learnings
--------------

* Battery 25C works for this drone.
* Better to use solder with lead, but prone to contamination (ensure proper storage).
* Check NRF24 radio communication first before assembling the drone. Always test parts individually.
* Keep it simple and complexity should not come first. Achieve minimum functionality for initial developments.
* 5V 16MHz Arduino Pro Mini is sufficient for the drone.
* Better to place MPU6050 away from electrical noise of the Arduino and other electrical components.
* Keep electrical components away from the conductive carbon fiber frame and insulate properly.
* Thin kapton tapes do not provide as much insulation as actual electrical tape. Though electrical tape is heavier. 
* Capacitors helps reduce power fluctuations and noise.

These are the lift of successful vendors that sold the components needed for this project that worked. 

- **Robojax**: Arduino Pro Mini 5V 16MHz 
- **HiLetGo**: NRF24L01+PA+LNA Transeiver Module
- **Aideepen**: NRF24L01 Transeiver Standard Module
- **Aideepen**: Arduino Nano Board
- **Fytoo**: 3.7V 600mAH LiON batteries - used for the controller
- **Fytoo**: 3.7V 220mAH LiON 25C battery
- **WWZMDiB**: Mini USB to TTL Serial Converter Adapter, 3.3V/5.5 V FTDI Breakout Board for Arduino
- **Wishiot**: Thumb Joystick Module Dual Axis Sensor for Arduino
- **DFRobot**: MPU6050 Gyro + Accelerometer Module
- **Digikey**: Surface mount components (resistors, diodes, n-channel MOSFETs, capacitors, etc)
- **Midzooparts**: Lilypad Buzzer Small Speaker Module for Arduino
- **Crazepony**: 4pcs 6x15mm Motor (Speed: Insane) 19000KV, 70000 RPM@3.7V

These are the list of vendors that sold components that did not work or were faulty for this project.

- **Hutomwua**: Arduino Pro Mini 5V 16MHz (Did not meet specifications or seems to be 10MHz on the backside - not advertised)
- **ELEGOO**: Arduno Nano (Faulty and did not program)
- **Fytoo**: 3.7V 1000mAH LiON batteries (potentially drew too much current and burned the board?)
- **Aideepen**: NRF24L01+PA+LNA Transeiver Module (Faulty and did not communicate)

