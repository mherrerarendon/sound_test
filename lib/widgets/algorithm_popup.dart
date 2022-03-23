import 'package:custom_pop_up_menu/custom_pop_up_menu.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:sound_test/blocs/tuner_bloc.dart';
import 'package:sound_test/models/detection_algorithm.dart';

class AlgorithmPopup extends StatelessWidget {
  AlgorithmPopup({Key? key}) : super(key: key);

  final CustomPopupMenuController _controller = CustomPopupMenuController();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<TunerBloc, TunerState>(
      buildWhen: (_, current) => current is AlgorithmChanged,
      builder: (context, state) {
        return CustomPopupMenu(
          child: Card(
            color: Theme.of(context).colorScheme.secondaryContainer,
            elevation: 18.0,
            child: Center(
              child: Padding(
                padding: const EdgeInsets.all(8.0),
                child: Text(
                  state.maybeWhen(
                      algorithmChanged: (algorithm) => algorithm.toName(),
                      orElse: () => ''),
                  style: const TextStyle(
                      fontSize: 20,
                      color: Colors.white,
                      fontWeight: FontWeight.normal),
                ),
              ),
            ),
          ),
          menuBuilder: () => ClipRRect(
            borderRadius: BorderRadius.circular(5),
            child: Container(
              color: const Color(0xFF4C4C4C),
              child: IntrinsicWidth(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: DetectionAlgorithm.values
                      .map(
                        (item) => GestureDetector(
                          behavior: HitTestBehavior.translucent,
                          onTap: () {
                            context
                                .read<TunerBloc>()
                                .add(TunerEvent.changeAlgorithm(item));
                            _controller.hideMenu();
                          },
                          child: Container(
                            height: 40,
                            padding: const EdgeInsets.symmetric(horizontal: 20),
                            child: Container(
                              margin: const EdgeInsets.only(left: 10),
                              padding: const EdgeInsets.symmetric(vertical: 10),
                              child: Text(
                                item.toName(),
                                style: const TextStyle(
                                  color: Colors.white,
                                  fontSize: 16,
                                ),
                              ),
                            ),
                          ),
                        ),
                      )
                      .toList(),
                ),
              ),
            ),
          ),
          pressType: PressType.singleClick,
          verticalMargin: -10,
          controller: _controller,
        );
      },
    );
  }
}
