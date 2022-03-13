import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';

class PitchPointer extends StatelessWidget {
  const PitchPointer(this._pitch, {Key? key}) : super(key: key);

  final Pitch _pitch;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, constraints) {
      final midX = constraints.maxWidth / 2;

      return Stack(
        children: [
          TweenAnimationBuilder<double>(
              tween: Tween<double>(
                  begin: 0,
                  end: _pitch.centsOffset / 100 * constraints.maxWidth + midX),
              duration: const Duration(milliseconds: 200),
              builder: (_, value, __) {
                return Positioned(
                  child: SizedBox(
                      child: FittedBox(
                          child: Column(
                        children: [
                          const Icon(Icons.arrow_upward),
                          Text(
                              '${_pitch.centsOffset > 0 ? '+' : ''}${_pitch.centsOffset.toStringAsFixed(2)}c')
                        ],
                      )),
                      width: constraints.maxHeight,
                      height: constraints.maxHeight),
                  left: value - constraints.maxHeight / 2,
                );
              }),
        ],
      );
    });
  }
}
