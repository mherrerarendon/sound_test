import 'package:flutter/material.dart';
import 'package:sound_test/widgets/text_box.dart';

class PartialDesc extends StatelessWidget {
  final String freq;
  final String intensity;
  const PartialDesc(this.freq, this.intensity, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(children: [
      Expanded(
        child: MyTextBox(freq),
      ),
      Expanded(
        child: MyTextBox(intensity),
      ),
    ]);
  }
}
