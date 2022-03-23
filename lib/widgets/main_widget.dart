import 'package:flutter/material.dart';
import 'package:sound_test/widgets/algorithm_popup.dart';
import 'package:sound_test/widgets/listen_widget.dart';
import 'package:sound_test/widgets/main_area.dart';
import 'package:url_launcher/url_launcher.dart';

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
        title: const Text('SondChek'),
        actions: [
          AlgorithmPopup(),
          IconButton(
              onPressed: () async {
                const url =
                    'https://github.com/mherrerarendon/sound_test#detection-algorithms';
                if (!await launch(url)) throw 'Could not launch $url';
              },
              icon: const Icon(Icons.help)),
        ],
      ),
      body: const MainArea(),
      floatingActionButton: const ListenWidget(),
    );
  }
}
