import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/tuner_inhereted_widget.dart';

class SelectAlgorithmPage extends StatelessWidget {
  const SelectAlgorithmPage({Key? key}) : super(key: key);

  Column buildListView(SettingsModel settings, BuildContext context) {
    final radioList = DetectionAlgorithm.values.map((algorithm) {
      return RadioListTile<DetectionAlgorithm>(
        title: Text(algorithm.toName()),
        groupValue: settings.detectionAlgorithm,
        value: algorithm,
        onChanged: (DetectionAlgorithm? val) async {
          settings.setDetectionAlgorithm(val!);
          await TunerInherited.of(context)!
              .tunerApi
              .setAlgorithm(algorithm: algorithm.toShortString());
          await Future.delayed(const Duration(milliseconds: 350));
          Navigator.pop(context);
        },
      );
    }).toList();
    return Column(
      children: radioList,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Pitch Detection Algorithm'),
      ),
      body: Consumer<SettingsModel>(
          builder: (context, settings, _) => buildListView(settings, context)),
    );
  }
}
