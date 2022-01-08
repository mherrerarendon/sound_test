import 'package:flutter/material.dart';

class PitchSlider extends StatefulWidget {
  const PitchSlider({Key? key}) : super(key: key);

  @override
  _PitchSliderState createState() => _PitchSliderState();
}

class _PitchSliderState extends State<PitchSlider> {
  ScrollController scrollController = ScrollController();

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        Container(
          // width: screenMaxWidth + 40,
          padding: EdgeInsets.only(left: 20.0, right: 2.0),
          height: 15,
          child: Center(
            child: MediaQuery.removePadding(
              context: context,
              removeBottom: true,
              removeLeft: true,
              removeRight: true,
              child: ListView.builder(
                  itemCount: 50,
                  controller: scrollController,
                  physics: NeverScrollableScrollPhysics(),
                  scrollDirection: Axis.horizontal,
                  itemBuilder: (context1, index) {
                    // return scale steps
                    return index == 50
                        ? SizedBox()
                        : Container(
                            width: 15,
                            alignment: Alignment.bottomLeft,
                            child: SizedBox(
                              child: Align(
                                alignment: Alignment.centerLeft,
                                child: Container(
                                  height: 10.0,
                                  width: 2,
                                  // color: (index % scaleStepLimit!) == 0
                                  //     ? widget.stepIndicatorColor
                                  //     : widget.stepIndicatorDividerColor,
                                ),
                              ),
                            ),
                          );
                  }),
            ),
          ),
        ),
        Container(
          // width: screenMaxWidth + 40,
          padding: EdgeInsets.only(left: 20.0, right: 2.0),
          height: 15,
          child: Center(
            child: MediaQuery.removePadding(
              context: context,
              removeBottom: true,
              removeLeft: true,
              removeRight: true,
              child: ListView.builder(
                  itemCount: 50,
                  controller: scrollController,
                  physics: NeverScrollableScrollPhysics(),
                  scrollDirection: Axis.horizontal,
                  itemBuilder: (context1, index) {
                    String scaleText;
                    //return scale text for cms or feet
                    scaleText = index.toString();

                    // find text padding to align text in center
                    double? textPadding = 5.0;
                    // }

                    return index == 50
                        ? SizedBox()
                        : Container(
                            width: 15,
                            padding: EdgeInsets.only(left: textPadding),
                            alignment: Alignment.bottomLeft,
                            child: Align(
                              alignment: Alignment.centerLeft,
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.center,
                                //mainAxisSize: MainAxisSize.min,
                                mainAxisAlignment: MainAxisAlignment.end,
                                children: [
                                  Align(
                                    alignment: Alignment.centerLeft,
                                    child: Text(
                                      scaleText,
                                      overflow: TextOverflow.ellipsis,
                                      style: TextStyle(
                                        letterSpacing: 1.0,
                                      ),
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
  }
}
