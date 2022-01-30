import 'package:flutter/material.dart';
import 'package:sound_test/widgets/played_pitch.dart';
import 'package:sound_test/widgets/cents_ruler.dart';
import 'package:sound_test/widgets/pitch_pointer.dart';

const double kWidth = 80;
const double kHeight = 80;

class MainDos extends StatelessWidget {
  const MainDos({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: const [
        Expanded(child: PlayedPitch()),
        SizedBox(height: 20),
        SizedBox(
          height: kHeight,
          child: CentsRuler(),
        ),
        SizedBox(
          height: kHeight,
          child: MyStatefulWidget(),
        ),
        SizedBox(height: 40),
      ],
    );
  }
}
