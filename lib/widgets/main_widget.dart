import 'package:provider/provider.dart';
import 'package:flutter/material.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/algorithm_details.dart';
import 'package:sound_test/widgets/algorithm_popup.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/main_area.dart';

class MainWidget extends StatefulWidget {
  const MainWidget({Key? key}) : super(key: key);

  @override
  _MainWidgetState createState() => _MainWidgetState();
}

class _MainWidgetState extends State<MainWidget> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Theme.of(context).colorScheme.background,
      appBar: AppBar(
        title: const Text('Sound Test'),
        actions: [
          AlgorithmPopup(),
          IconButton(
              onPressed: () {
                final settings =
                    Provider.of<SettingsModel>(context, listen: false);
                Navigator.push(
                  context,
                  MaterialPageRoute(
                      builder: (context) =>
                          AlgorithmDetails(settings.detectionAlgorithm.index)),
                );
              },
              icon: const Icon(Icons.help)),
        ],
      ),
      body: const MainArea(),
      floatingActionButton: const ListenWidget(),
    );
  }
}
