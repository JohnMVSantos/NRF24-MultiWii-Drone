.. _prototypes:

Prototypes
===========

This page will provide an overview of the design of the prototypes 
that will start with the initial prototype and the reasons behind the design changes that lead to the next prototype.

Protoype One
-------------

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
    - Using 2 blade (faster) 37mm propellers (Use 4 blade propellers for more thrust)

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