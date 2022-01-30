import 'package:flutter/material.dart';

class Tick extends StatelessWidget {
  const Tick(this.width, this.height, this.color, {Key? key}) : super(key: key);
  final double height;
  final double width;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, constraints) {
      return SizedBox(
        child: Align(
          alignment: Alignment.center,
          child: Container(
            width: this.width,
            height: this.height * constraints.maxHeight,
            color: this.color,
          ),
        ),
      );
    });
  }
}
