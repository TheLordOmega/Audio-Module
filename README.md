Audio Module 
A module that can play sound over 3.5mm , speaker with adjustable volume with a knob
Key features:
- a knob
- a 2w speaker
- 3.5mm jack
- no storage

## PCB

![[Pasted image 20261001231218.png]]

## Schematic
![[Pasted image 20261001231232.png]]

## 3D Case

![[Pasted image 20261001231628.png]]
## Bill of Materials (excluding console)

Also found in [bom.csv](./bom.csv).

| Item                                                  | Price per unit | Nr of units | Total price | Link                                               |
| ----------------------------------------------------- | -------------- | ----------- | ----------- | -------------------------------------------------- |
| PCB                                                   |                | 1           | 10$         | -                                                  |
| 2x7 2.54mm pin header (J1 breakout board)             | ~$0.20-0.39    | 1           | ~$0.20-0.39 | https://www.aliexpress.com/item/4000186187780.html |
| 3-pin flying-lead connector (JST-PH or Dupont header) |                | 1           | -           | -                                                  |
| MD0/MD1 ID resistors, 0603                            |                | 2           | -           | -                                                  |
| a volume knob                                         |                | 1           | -           | -                                                  |
|                                                       |                |             |             |                                                    |
| **Total**                                             |                |             | 10$         |                                                    |

## Credits

* Driver: [`xpanse_api`](https://docs.rs/xpanse-api)
* Thanks to Hack Club and the Hackxpansion team for the console platform this module plugs into: https://github.com/hackclub/hackxpansion
