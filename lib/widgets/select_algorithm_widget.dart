import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/tuner_inhereted_widget.dart';

class SelectAlgorithmPage extends StatelessWidget {
  const SelectAlgorithmPage({Key? key}) : super(key: key);

  ListView buildListView(SettingsModel settings, BuildContext context) {
    final radioList = DetectionAlgorithm.values.map((algorithm) {
      return RadioListTile<DetectionAlgorithm>(
        title: Text(algorithm.toName()),
        groupValue: settings.detectionAlgorithm,
        value: algorithm,
        onChanged: (DetectionAlgorithm? val) {
          settings.setDetectionAlgorithm(val!);
          TunerInherited.of(context)!
              .tunerApi
              .setAlgorithm(algorithm: algorithm.toShortString());
        },
      );
    }).toList();
    return ListView(
      children: radioList,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Select Pitch Detection Algorithm'),
      ),
      body: Consumer<SettingsModel>(
          builder: (context, settings, _) => buildListView(settings, context)),
    );
  }
}
