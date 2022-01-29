import 'package:flutter/material.dart';

class CentsRuler extends StatelessWidget {
  const CentsRuler({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
        builder: (BuildContext context, BoxConstraints constraints) {
      final int numTicks = constraints.maxWidth ~/ 20 * 2 + 1;
      return Row(children: ,);
    });
  }
}
