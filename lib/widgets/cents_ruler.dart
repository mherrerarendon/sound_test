import 'package:flutter/material.dart';
import 'package:sound_test/widgets/tick.dart';

class CentsRuler extends StatelessWidget {
  const CentsRuler({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
        builder: (BuildContext context, BoxConstraints constraints) {
      final int numTicks = constraints.maxWidth ~/ 20 ~/ 4 * 4 + 1;
      List<Widget> ticks = List.generate(numTicks, (index) {
        final width = index == numTicks ~/ 2
            ? 4.0
            : index % 2 == 0
                ? 3.0
                : 2.0;
        final height = index == numTicks ~/ 2
            ? 1.0
            : index % 2 == 0
                ? .6
                : .3;
        return Expanded(child: Tick(width, height, Colors.black));
      });
      return Row(children: ticks);
    });
  }
}
