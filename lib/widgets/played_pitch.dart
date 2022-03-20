import 'package:flutter/material.dart';
import 'package:sound_test/api.dart';

const double kWidth = 40;

class PlayedPitch extends StatelessWidget {
  const PlayedPitch(this._noteResult, {Key? key}) : super(key: key);

  final NoteResult _noteResult;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        SizedBox(
          width: kWidth,
          child: FittedBox(
            child: Text(
              _noteResult.previousNoteName,
              style: TextStyle(
                  color: _noteResult.inTune
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
              _noteResult.noteName,
              style: TextStyle(
                  color: _noteResult.inTune
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
              _noteResult.nextNoteName,
              style: TextStyle(
                  color: _noteResult.inTune
                      ? Theme.of(context).colorScheme.onSurface
                      : Colors.grey),
            ),
          ),
        ),
      ],
    );
  }
}
