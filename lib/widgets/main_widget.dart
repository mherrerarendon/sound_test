import 'package:flutter/material.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/pitch_slider.dart';
import 'package:sound_test/widgets/all_partials.dart';

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
      body: makeBody(),
      floatingActionButton: const ListenWidget(),
    );
  }
}
