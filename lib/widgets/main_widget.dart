import 'package:flutter/material.dart';
import 'package:sound_test/widgets/algorithm_button.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/pitch_slider.dart';
import 'package:sound_test/widgets/all_partials.dart';
import 'package:sound_test/widgets/played_pitch.dart';

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
          const SizedBox(height: 40),
          const PlayedPitch(),
          const SizedBox(height: 20),
          PitchSlider(),
          const SizedBox(height: 20),
          const AllPartials(),
          const SizedBox(height: 30),
        ],
      );
    }

    return Scaffold(
      backgroundColor: Theme.of(context).colorScheme.background,
      appBar: AppBar(
        title: const Text('Sound Test'),
      ),
      // drawer: const AppDrawer(),
      body: makeBody(),
      bottomNavigationBar: BottomAppBar(
        shape: const CircularNotchedRectangle(),
        child: Container(
          height: 120.0,
          child: const Padding(
            padding: EdgeInsets.only(top: 30.0),
            child: AlgorithmButton(),
          ),
        ),
      ),
      floatingActionButton: const ListenWidget(),
      floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
    );
  }
}
