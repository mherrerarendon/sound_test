import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';

const double kWidth = 80;

class PlayedPitch extends StatelessWidget {
  const PlayedPitch({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
        builder: (BuildContext context, BoxConstraints constraints) {
      final double height = constraints.maxHeight;
      return Consumer<PartialsModel>(
        builder: (context, partials, _) {
          return SizedBox(
              height: height,
              child: Row(
                // crossAxisAlignment: CrossAxisAlignment.center,
                // mainAxisSize: MainAxisSize.max,
                // mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  Container(
                    width: kWidth,
                    alignment: Alignment.bottomLeft,
                    child: Text(
                      partials.leftNoteName,
                      style: TextStyle(
                          height: 0,
                          fontSize: height * .1,
                          color: partials.inTune()
                              ? Theme.of(context).colorScheme.onSurface
                              : Colors.grey),
                    ),
                  ),
                  Expanded(
                    child: Container(
                      alignment: Alignment.bottomCenter,
                      child: Text(
                        partials.noteName,
                        style: TextStyle(
                            height: 0,
                            fontSize: height * .6,
                            color: partials.inTune()
                                ? Theme.of(context).colorScheme.onSurface
                                : Colors.grey),
                      ),
                    ),
                  ),
                  Container(
                    width: kWidth,
                    alignment: Alignment.bottomRight,
                    child: Text(
                      partials.rigthNoteName,
                      style: TextStyle(
                          height: 0,
                          fontSize: height * .1,
                          color: partials.inTune()
                              ? Theme.of(context).colorScheme.onSurface
                              : Colors.grey),
                    ),
                  ),
                ],
              ));
        },
      );
    });
  }
}
