import 'package:provider/provider.dart';
import 'package:flutter/material.dart';
import 'package:sound_test/blocs/tuner_bloc.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/algorithm_details.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/main_area.dart';

class MainWidget extends StatefulWidget {
  const MainWidget({Key? key}) : super(key: key);

  @override
  _MainWidgetState createState() => _MainWidgetState();
}

GlobalKey _keyAlgButton = GlobalKey();

class _MainWidgetState extends State<MainWidget> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Theme.of(context).colorScheme.background,
      appBar: AppBar(
        title: const Text('Sound Test'),
        actions: [
          Container(
            key: _keyAlgButton,
            child: Builder(builder: (context) {
              final settings =
                  Provider.of<SettingsModel>(context, listen: false);
              final width = MediaQuery.of(context).size.width;
              final height = MediaQuery.of(context).size.height;
              return ElevatedButton(
                  onPressed: () async {
                    await showMenu(
                        initialValue: settings.detectionAlgorithm,
                        context: context,
                        position: RelativeRect.fromLTRB(
                            width - 20, height - 20, width, height),
                        items: DetectionAlgorithm.values.map((algorithm) {
                          return PopupMenuItem(
                            value: algorithm,
                            child: Text(algorithm.toName()),
                          );
                        }).toList());
                  },
                  child: Text(settings.detectionAlgorithm.toName(),
                      style: const TextStyle(fontSize: 24)));
            }),
          ),
          PopupMenuButton(onSelected: (DetectionAlgorithm algorithm) {
            context
                .read<TunerBloc>()
                .add(TunerEvent.changeAlgorithm(algorithm.toShortString()));
          }, itemBuilder: (context) {
            return DetectionAlgorithm.values.map((algorithm) {
              return PopupMenuItem(
                value: algorithm,
                child: Text(algorithm.toName()),
              );
            }).toList();
          }),
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
