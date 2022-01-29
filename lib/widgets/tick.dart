import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';

class Tick extends StatelessWidget {
  const Tick(this.width, this.color, {Key? key}) : super(key: key);
  final double width;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return Container(
      child: SizedBox(
        child: Align(
          alignment: Alignment.center,
          child: Container(
            width: this.width,
            color: this.color,
          ),
        ),
      ),
    );
  }
}
