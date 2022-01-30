import 'package:flutter/material.dart';
import 'package:sound_test/widgets/played_pitch.dart';
import 'package:sound_test/widgets/cents_ruler.dart';
import 'package:sound_test/widgets/pitch_pointer.dart';

class MainDos extends StatelessWidget {
  const MainDos({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: const [
        Expanded(flex: 5, child: PlayedPitch()),
        SizedBox(height: 20),
        Expanded(
          flex: 1,
          child: CentsRuler(),
        ),
        Expanded(
          flex: 2,
          child: PitchPointer(),
        ),
        SizedBox(height: 40),
      ],
    );
  }
}
