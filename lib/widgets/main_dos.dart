import 'package:flutter/material.dart';
import 'package:sound_test/widgets/played_pitch.dart';

const double kWidth = 80;
const double kHeight = 80;

class MainDos extends StatelessWidget {
  const MainDos({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(child: PlayedPitch()),
        Container(
          height: kHeight,
          color: Colors.blue,
        ),
        Container(
          height: kHeight,
          color: Colors.green,
        ),
      ],
    );
    return Row(
      children: [
        Container(
          width: kWidth,
          child: Column(
            children: [
              Expanded(
                  child: Container(
                color: Colors.purple,
              )),
              Container(
                height: kHeight,
                color: Colors.blue,
              ),
              Container(
                height: kHeight,
                color: Colors.green,
              ),
            ],
          ),
        ),
        Expanded(
            child: Column(
          children: [
            Expanded(
                child: Container(
              color: Colors.green,
            )),
            Container(
              height: kHeight,
              color: Colors.purple,
            ),
            Container(
              height: kHeight,
              color: Colors.blue,
            )
          ],
        )),
        Container(
          width: kWidth,
          color: Colors.red,
        ),
      ],
    );
  }
}
