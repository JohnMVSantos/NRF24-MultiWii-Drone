.. _prototypes:

Prototypes
===========

This page will provide an overview of the design of the prototypes 
that will start with the initial prototype and the reasons behind the design changes that lead to the next prototype.

Protoype 1
------------

.. image:: assets/prototype_1.jpg
   :width: 300px
   :align: center
   :alt: Prototype One.

|

The first protoype is the initial attempt at building the NRF24-MultiWii-Drone. However, there were weaknesses in the design that led to failure. 

**The main issues are listed below:**

1. Too heavy
    - Motor fasteners that were improvised using drywall anchors were too heavy (Motors will be glued directly in the next prototype).
    - The wires were too thick using 24AWG.
    - The perforated board was too large with lots of unused space.
    - The soldered pin connectors (NRF24L01, perforated board, buzzer) are too heavy (These will be removed and wires will be soldered directly in the next prototype).
    - 1N5819 diodes are too heavy (Using 1N4148 surface mount diodes in the next prototype).
    - Using velcro is too heavy (Use super glue instead).

2. Small Propellers/Less Thrust
    - Using 2 blade (faster) 37mm propellers (Use 4 blade propellers for more thrust).

3. Conductive Carbon Fiber Frame
    - Possible short circuits when mounted on the carbon fiber frame (Use kapton tape for better insulation with the electrical components).

4. Poor Solder Connections
    - Damaged solder tips (oxidized) resulted in poor solder connections with possible decrease in conductivity and connectivity between components and possible short circuits.
    - Replace solder tips and properlly resolder the connections in the next prototype.

5. Need Soldered Controller with Enclosure
    - The controller uses a breadboard with weak connections and unmanaged wiring. Requires a proper enclosure with soldered connections for better reliability.

**Additional materials and replacement needed for the next prototype:**

1. Arduino Pro Mini Atmega 328P 5V/16MHz (un-soldered)
2. MPU6050 from DFRobot (un-soldered)
3. Motor Encoder Circuit
    - Surface mount 1N4148 diodes
    - Solder tips + high quality thin solder 
4. Larger 4 blade propellers
5. Kapton tape for insulation
6. Copper Sheet for proper grounding

Protoype 1.1
------------

.. image:: assets/prototype_1.1.jpg
   :width: 300px
   :align: center
   :alt: Prototype One.

|

The second protoype is the second attempt at building the NRF24-MultiWii-Drone. However, there were still weaknesses in the design that led to failure. 

**The main issues are listed below:**

1. Kapton Tape Does not Provide Proper Insulation
    - After adding one layer of kapton tape, it seemed that the electrical components, mainly the IMU shorted and was damaged. This resulted in the gyro unable to properly register the movements in MultiSim. The next prototype will include a proper enclosure for the electrical components of the drone. Although the main frame will still be carbon fiber, a separate box enclsoure will be used to cover the electrical components properly using light materials. 
    - Furthermore, investigating the user of hot glue for proper insulation. However, initial research suggests this may lead to further design failures.

2. Controller is Too Complex
    - "Simplicity is the ultimate sophistication." - Leonardo da Vinci 
    - Unnessary components in the controller atleast for the MVP such as the 16x2 LCD to track voltage and the potentiometer should be removed. The controller should be simplified to only include the basic components for communicating with the drone and this includes the Arduino Nano controller, the radio module + PA + LNA components, the two joysticks, the two SPDT switches, and the SPST switch with the 3.7V batteries. 

3. Controller Battery Has Too Much Current
    - Prolonged use of the controller lead to overheating of the components and controller failure. Theory is that the battery packs too much current which the components could not handle resulting in breakdown. The two batteries are connected in parallel which are 3.7V 1000mAH. Looking into the use of 3.7V and 600mAH batteries instead. 

4. Remove the Grounded Copper Sheet
    - This may not be needed as I have not encountered any issues with the drone resetting. This solution was suggested online, but I should not implement solutions to problems that does not exist in my design.

**Additional materials and replacement needed for the next prototype:**

1. Arduino Pro Mini Atmega 328P 5V/16MHz (un-soldered)
2. Arduino Nano 
3. Radio Modules NRF24L01 + PA + LNA
4. 3.7V 600mAH batteries (2x)
