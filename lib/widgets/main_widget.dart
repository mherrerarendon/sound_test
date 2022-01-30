import 'package:flutter/material.dart';
import 'package:sound_test/widgets/algorithm_button.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/main_dos.dart';

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
      ),
      body: const MainDos(),
      bottomNavigationBar: const BottomAppBar(
        shape: CircularNotchedRectangle(),
        child: SizedBox(
          height: 120.0,
          child: Padding(
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
