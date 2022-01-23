import 'package:flutter/material.dart';
import 'package:sound_test/widgets/algorithm_button.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/pitch_slider.dart';
import 'package:sound_test/widgets/all_partials.dart';
import 'package:sound_test/widgets/drawer_widget.dart';
import 'package:sound_test/widgets/tuner_inhereted_widget.dart';

class MainWidget extends StatefulWidget {
  const MainWidget({Key? key}) : super(key: key);

  @override
  _MainWidgetState createState() => _MainWidgetState();
}

class _MainWidgetState extends State<MainWidget> {
  @override
  Widget build(BuildContext context) {
    Widget makeBody() {
      return Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16.0),
            child: const AlgorithmButton(),
          ),
          const SizedBox(height: 20),
          PitchSlider(),
          const SizedBox(height: 20),
          const AllPartials(),
        ],
      );
    }

    return Scaffold(
      backgroundColor: Theme.of(context).colorScheme.background,
      appBar: AppBar(
        title: const Text('Sound Test'),
      ),
      drawer: const AppDrawer(),
      body: makeBody(),
      floatingActionButton: const ListenWidget(),
    );
  }
}
