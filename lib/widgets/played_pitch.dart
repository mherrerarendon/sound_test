import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';

const double kWidth = 40;

class PlayedPitch extends StatelessWidget {
  const PlayedPitch(this._pitch, {Key? key}) : super(key: key);

  final Pitch _pitch;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        SizedBox(
          width: kWidth,
          child: FittedBox(
            child: Text(
              _pitch.previousNoteName,
              style: TextStyle(
                  color: _pitch.inTune
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
              _pitch.noteName,
              style: TextStyle(
                  color: _pitch.inTune
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
              _pitch.nextNoteName,
              style: TextStyle(
                  color: _pitch.inTune
                      ? Theme.of(context).colorScheme.onSurface
                      : Colors.grey),
            ),
          ),
        ),
      ],
    );
  }
}
