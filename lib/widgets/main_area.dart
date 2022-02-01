import 'package:flutter/material.dart';
import 'package:sound_test/widgets/played_pitch.dart';
import 'package:sound_test/widgets/cents_ruler.dart';
import 'package:sound_test/widgets/pitch_pointer.dart';
import 'package:sound_test/widgets/debug_partial_desc.dart';

const bool _debug = false;

class MainArea extends StatelessWidget {
  const MainArea({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        _debug ? const DebugPartialDesc() : Container(),
        const Expanded(flex: 5, child: PlayedPitch()),
        const SizedBox(height: 20),
        const Expanded(
          flex: 1,
          child: CentsRuler(),
        ),
        const Expanded(
          flex: 2,
          child: PitchPointer(),
        ),
        const SizedBox(height: 40),
      ],
    );
  }
}
