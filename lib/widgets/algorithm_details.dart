import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/blocs/tuner_bloc.dart';
import 'package:sound_test/models/settings_model.dart';
import 'package:sound_test/widgets/algorithm_details_page.dart';
import 'package:page_view_indicators/circle_page_indicator.dart';

class AlgorithmDetails extends StatefulWidget {
  const AlgorithmDetails(this.initialPage, {Key? key}) : super(key: key);
  final int initialPage;

  @override
  State<AlgorithmDetails> createState() => _AlgorithmDetailsState(initialPage);
}

class _AlgorithmDetailsState extends State<AlgorithmDetails> {
  _AlgorithmDetailsState(int initialPage)
      : _pageController = PageController(initialPage: initialPage),
        _currentPageNotifier = ValueNotifier<int>(initialPage);
  final PageController _pageController;
  final ValueNotifier<int> _currentPageNotifier;

  _buildPageView() {
    return PageView.builder(
        itemCount: DetectionAlgorithm.values.length,
        controller: _pageController,
        itemBuilder: (BuildContext context, int index) {
          return AlgorithmDetailsPage(DetectionAlgorithm.values[index]);
        },
        onPageChanged: (int index) {
          _currentPageNotifier.value = index;
        });
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
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
        title: const Text('Algorithm Details'),
      ),
      body: Column(children: [
        Expanded(child: _buildPageView()),
        _buildCircleIndicator(),
        const SizedBox(height: 16),
        FittedBox(
          child: ElevatedButton(
            onPressed: () async {
              final algorithm =
                  DetectionAlgorithm.values[_pageController.page!.round()];
              settings.setDetectionAlgorithm(algorithm);
              context
                  .read<TunerBloc>()
                  .add(TunerEvent.changeAlgorithm(algorithm.toShortString()));
              Navigator.pop(context);
            },
            child: const Text('Select', style: TextStyle(fontSize: 30)),
          ),
        ),
        const SizedBox(height: 16),
      ]),
    );
  }
}
