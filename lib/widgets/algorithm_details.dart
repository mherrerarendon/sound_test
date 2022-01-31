import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/tuner_inhereted_widget.dart';
import 'package:sound_test/widgets/algorithm_details_page.dart';
import 'package:page_view_indicators/circle_page_indicator.dart';

class AlgorithmDetails extends StatefulWidget {
  AlgorithmDetails({Key? key}) : super(key: key);

  @override
  State<AlgorithmDetails> createState() => _AlgorithmDetailsState();
}

class _AlgorithmDetailsState extends State<AlgorithmDetails> {
  late PageController _pageController = PageController();
  late ValueNotifier<int> _currentPageNotifier = ValueNotifier<int>(0);

  @override
  void initState() {
    super.initState();
    final SettingsModel settings = Provider.of<SettingsModel>(context);
    _currentPageNotifier =
        ValueNotifier<int>(settings.detectionAlgorithm.index);
    _pageController =
        PageController(initialPage: settings.detectionAlgorithm.index);
  }

  _buildPageView() {
    return Container(
      child: PageView.builder(
          itemCount: DetectionAlgorithm.values.length,
          controller: _pageController,
          itemBuilder: (BuildContext context, int index) {
            return AlgorithmDetailsPage(DetectionAlgorithm.values[index]);
          },
          onPageChanged: (int index) {
            _currentPageNotifier.value = index;
          }),
    );
  }

  _buildCircleIndicator() {
    return CirclePageIndicator(
      itemCount: DetectionAlgorithm.values.length,
      currentPageNotifier: _currentPageNotifier,
    );
  }

  @override
  Widget build(BuildContext context) {
    final SettingsModel settings = Provider.of<SettingsModel>(context);

    return Scaffold(
      appBar: AppBar(
        title: Text('Algorithm Details'),
      ),
      body: Column(children: [
        Expanded(child: _buildPageView()),
        _buildCircleIndicator(),
        Container(
          child: FittedBox(
            child: ElevatedButton(
              onPressed: () async {
                final algorithm =
                    DetectionAlgorithm.values[_pageController.page!.round()];
                settings.setDetectionAlgorithm(algorithm);
                await TunerInherited.of(context)!
                    .tunerApi
                    .setAlgorithm(algorithm: algorithm.toShortString());
                Navigator.pop(context);
                Navigator.pop(context);
              },
              child: const Text('Select', style: TextStyle(fontSize: 30)),
            ),
          ),
        ),
      ]),
    );
  }
}
