import 'package:flutter/material.dart';
import 'package:sound_test/models/settings_model.dart';

class AlgorithmDetailsPage extends StatelessWidget {
  const AlgorithmDetailsPage(this._algorithm, {Key? key}) : super(key: key);
  final DetectionAlgorithm _algorithm;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
            flex: 1,
            child: FittedBox(
              child: Text(_algorithm.toName(),
                  style: Theme.of(context).textTheme.headline6),
            )),
        Expanded(
          flex: 3,
          child: Container(
            padding: const EdgeInsets.all(8.0),
            child: Text(
              _algorithm.description(),
              style: const TextStyle(fontSize: 30),
            ),
          ),
        ),
        const SizedBox(height: 10),
        Expanded(
            flex: 2,
            child: Container(
              decoration: BoxDecoration(
                  border: Border.all(
                    color: Theme.of(context).colorScheme.background,
                  ),
                  borderRadius: const BorderRadius.all(Radius.circular(20))),
              child: FittedBox(
                child: Text(
                  _algorithm.instruments(),
                  style: const TextStyle(
                      fontSize:
                          400), // The larger the fontsize the less pixelated the images look.
                ),
              ),
            )),
        const Spacer(),
      ],
    );
  }
}
