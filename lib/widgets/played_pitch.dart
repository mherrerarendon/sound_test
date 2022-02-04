import 'package:flutter/material.dart';
import 'package:sound_test/models/partials_model.dart';

const double kWidth = 40;

class PlayedPitch extends StatelessWidget {
  const PlayedPitch(this._partialsModel, {Key? key}) : super(key: key);

  final PartialsModel _partialsModel;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        SizedBox(
          width: kWidth,
          child: FittedBox(
            child: Text(
              _partialsModel.leftNoteName,
              style: TextStyle(
                  color: _partialsModel.inTune()
                      ? Theme.of(context).colorScheme.onSurface
                      : Colors.grey),
            ),
          ),
        ),
        const SizedBox(
          width: 10,
        ),
        Expanded(
          child: FittedBox(
            alignment: Alignment.bottomCenter,
            child: Text(
              _partialsModel.noteName,
              style: TextStyle(
                  color: _partialsModel.inTune()
                      ? Theme.of(context).colorScheme.onSurface
                      : Colors.grey),
            ),
          ),
        ),
        const SizedBox(
          width: 10,
        ),
        SizedBox(
          width: kWidth,
          child: FittedBox(
            child: Text(
              _partialsModel.rigthNoteName,
              style: TextStyle(
                  color: _partialsModel.inTune()
                      ? Theme.of(context).colorScheme.onSurface
                      : Colors.grey),
            ),
          ),
        ),
      ],
    );
  }
}
