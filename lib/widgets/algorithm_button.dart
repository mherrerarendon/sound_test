import 'package:flutter/material.dart';
import 'package:sound_test/widgets/select_algorithm_widget.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';

class AlgorithmButton extends StatelessWidget {
  const AlgorithmButton({Key? key}) : super(key: key);

  void _pushSelectAlgorithm(BuildContext context) {
    showModalBottomSheet(
        context: context,
        builder: (context) {
          return const SelectAlgorithmPage();
        });
  }

  @override
  Widget build(BuildContext context) {
    return FittedBox(
      child: ElevatedButton(
        child: Consumer<SettingsModel>(
            builder: (context, settings, _) => Text(
                  settings.detectionAlgorithm.toName(),
                  style: const TextStyle(),
                )),
        onPressed: () {
          _pushSelectAlgorithm(context);
        },
      ),
    );
  }
}
