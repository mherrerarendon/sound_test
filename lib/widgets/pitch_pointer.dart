import 'package:flutter/material.dart';
import 'package:sound_test/models/partials_model.dart';

class PitchPointer extends StatelessWidget {
  const PitchPointer(this._partialsModel, {Key? key}) : super(key: key);

  final PartialsModel _partialsModel;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, constraints) {
      final midX = constraints.maxWidth / 2;

      return Stack(
        children: [
          TweenAnimationBuilder<double>(
              tween: Tween<double>(
                  begin: 0,
                  end: _partialsModel.centsOffset / 100 * constraints.maxWidth +
                      midX),
              duration: const Duration(milliseconds: 200),
              builder: (_, value, __) {
                return Positioned(
                  child: SizedBox(
                      child: FittedBox(
                          child: Column(
                        children: [
                          const Icon(Icons.arrow_upward),
                          Text(
                              '${_partialsModel.centsOffset > 0 ? '+' : ''}${_partialsModel.centsOffset.toStringAsFixed(2)}c')
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
