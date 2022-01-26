import 'package:flutter/material.dart';
import 'package:sound_test/models/partials_model.dart';
import 'package:provider/provider.dart';

const kCellWidth = 150.0;
const kCellHeight = 30.0;
const kNumCells = 12 * 8 + 2; // Notes in regular piano
const kInTuneWidth = kMaxCentsOffset * 2 / 100 * kCellWidth;

const kA4ScrollOffset = kA4Index * kCellWidth;

class PitchSlider extends StatelessWidget {
  PitchSlider({Key? key}) : super(key: key);
  final ScrollController _scrollController = ScrollController();

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
        builder: (BuildContext context, BoxConstraints constraints) {
      final fftPeakModel = Provider.of<PartialsModel>(context);
      scrollTo() {
        final stepsFromA4 = fftPeakModel.stepsFromA4;
        if (stepsFromA4 + kA4Index >= 0) {
          final offset = kA4ScrollOffset +
              stepsFromA4 * kCellWidth -
              (constraints.maxWidth / 2) +
              (kCellWidth / 2);
          _scrollController.animateTo(offset,
              duration: const Duration(milliseconds: 100),
              curve: Curves.linear);
        }
      }

      WidgetsBinding.instance?.addPostFrameCallback((_) => scrollTo());
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          SizedBox(
            child: Align(
              alignment: Alignment.center,
              child: Container(
                height: 10.0,
                width: kInTuneWidth,
                color: fftPeakModel.inTune()
                    ? Theme.of(context).colorScheme.onSurface
                    : Colors.grey,
              ),
            ),
          ),
          SizedBox(
            height: kCellHeight,
            child: Center(
              child: MediaQuery.removePadding(
                context: context,
                removeBottom: true,
                removeLeft: true,
                removeRight: true,
                child: ListView.builder(
                    itemCount: kNumCells,
                    controller: _scrollController,
                    physics: const NeverScrollableScrollPhysics(),
                    scrollDirection: Axis.horizontal,
                    itemBuilder: (context1, index) {
                      return index == kNumCells
                          ? const SizedBox()
                          : Container(
                              width: kCellWidth,
                              alignment: Alignment.bottomLeft,
                              child: SizedBox(
                                child: Align(
                                  alignment: Alignment.center,
                                  child: Container(
                                    height: 10.0,
                                    width: 2,
                                    color: fftPeakModel.inTune()
                                        ? Theme.of(context)
                                            .colorScheme
                                            .primaryVariant
                                        : Theme.of(context)
                                            .colorScheme
                                            .secondaryVariant,
                                  ),
                                ),
                              ),
                            );
                    }),
              ),
            ),
          ),
          SizedBox(
            height: kCellHeight,
            child: Center(
              child: MediaQuery.removePadding(
                context: context,
                removeBottom: true,
                removeLeft: true,
                removeRight: true,
                child: ListView.builder(
                    itemCount: kNumCells,
                    controller: _scrollController,
                    physics: const NeverScrollableScrollPhysics(),
                    scrollDirection: Axis.horizontal,
                    itemBuilder: (context1, index) {
                      final octave = ((index + 9) ~/ 12)
                          .toString(); // plus 9 because we are starting at A0
                      final noteName = kNotes[index % 12];
                      final scaleText = '$noteName$octave';

                      return index == kNumCells
                          ? const SizedBox()
                          : Container(
                              width: kCellWidth,
                              alignment: Alignment.bottomLeft,
                              child: Align(
                                alignment: Alignment.centerLeft,
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.center,
                                  mainAxisAlignment: MainAxisAlignment.end,
                                  children: [
                                    Align(
                                      alignment: Alignment.center,
                                      child: Text(
                                        scaleText,
                                        overflow: TextOverflow.ellipsis,
                                        style: TextStyle(
                                            letterSpacing: 1.0,
                                            fontSize: 24,
                                            color: Theme.of(context)
                                                .colorScheme
                                                .secondaryVariant),
                                      ),
                                    )
                                  ],
                                ),
                              ),
                            );
                    }),
              ),
            ),
          ),
        ],
      );
    });
  }
}
